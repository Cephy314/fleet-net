//! I/O test helpers for simulating various network conditions

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::time::Sleep;

/// A reader that introduces artificial delays between reads to simulate slow networks.
pub struct SlowReader<R> {
    inner: R,
    delay: Duration,
    sleep: Option<Pin<Box<Sleep>>>,
}

impl<R> SlowReader<R> {
    /// Create a new slow reader with the specified delay between reads.
    pub fn new(inner: R, delay: Duration) -> Self {
        Self {
            inner,
            delay,
            sleep: None,
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for SlowReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // If we have a pending sleep, poll it first
        if let Some(sleep) = self.sleep.as_mut() {
            match sleep.as_mut().poll(cx) {
                Poll::Ready(_) => {
                    self.sleep = None;
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        // Now try to read from the inner stream
        let result = Pin::new(&mut self.inner).poll_read(cx, buf);

        // If we read something, set up a delay for the next read
        if matches!(result, Poll::Ready(Ok(_))) && !buf.filled().is_empty() {
            self.sleep = Some(Box::pin(tokio::time::sleep(self.delay)));
        }

        result
    }
}

/// A writer that introduces artificial delays between writes to simulate slow networks.
pub struct SlowWriter<W> {
    inner: W,
    delay: Duration,
    sleep: Option<Pin<Box<Sleep>>>,
}

impl<W> SlowWriter<W> {
    /// Create a new slow writer with the specified delay between writes.
    pub fn new(inner: W, delay: Duration) -> Self {
        Self {
            inner,
            delay,
            sleep: None,
        }
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for SlowWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        // If we have a pending sleep, poll it first
        if let Some(sleep) = self.sleep.as_mut() {
            match sleep.as_mut().poll(cx) {
                Poll::Ready(_) => {
                    self.sleep = None;
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        // Now try to write to the inner stream
        let result = Pin::new(&mut self.inner).poll_write(cx, buf);

        // If we wrote something, set up a delay for the next write
        if matches!(result, Poll::Ready(Ok(n)) if n > 0) {
            self.sleep = Some(Box::pin(tokio::time::sleep(self.delay)));
        }

        result
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// A stream that can be disrupted to simulate connection failures.
pub struct DisruptableStream<S> {
    inner: Option<S>,
}

impl<S> DisruptableStream<S> {
    /// Create a new disruptable stream.
    pub fn new(inner: S) -> Self {
        Self { inner: Some(inner) }
    }

    /// Disrupt the stream, simulating a connection drop.
    pub fn disrupt(&mut self) {
        self.inner = None;
    }

    /// Check if the stream has been disrupted.
    pub fn is_disrupted(&self) -> bool {
        self.inner.is_none()
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for DisruptableStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match self.inner.as_mut() {
            Some(inner) => Pin::new(inner).poll_read(cx, buf),
            None => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection disrupted",
            ))),
        }
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for DisruptableStream<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.inner.as_mut() {
            Some(inner) => Pin::new(inner).poll_write(cx, buf),
            None => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection disrupted",
            ))),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.inner.as_mut() {
            Some(inner) => Pin::new(inner).poll_flush(cx),
            None => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection disrupted",
            ))),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.inner.as_mut() {
            Some(inner) => Pin::new(inner).poll_shutdown(cx),
            None => Poll::Ready(Ok(())), // Already disrupted, consider it shutdown
        }
    }
}
