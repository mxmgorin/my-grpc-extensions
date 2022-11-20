use std::{fmt::Debug, pin::Pin, time::Duration};

use tokio::sync::mpsc::error::SendTimeoutError;
const DEFAULT_SEND_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn create_empty_stream<TDest>() -> Result<
    tonic::Response<
        Pin<
            Box<
                dyn tonic::codegen::futures_core::Stream<Item = Result<TDest, tonic::Status>>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    tonic::Status,
>
where
    TDest: Send + Sync + Debug + 'static,
{
    let (_tx, rx) = tokio::sync::mpsc::channel(1);

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let response: Pin<
        Box<dyn futures::Stream<Item = Result<TDest, tonic::Status>> + Send + Sync + 'static>,
    > = Box::pin(output_stream);
    return Ok(tonic::Response::new(response));
}

pub async fn send_signle_item_to_stream<TDest>(
    item: TDest,
    #[cfg(feature = "adjust-server-stream")] send_timeout: Duration,
) -> Result<
    tonic::Response<
        Pin<
            Box<
                dyn tonic::codegen::futures_core::Stream<Item = Result<TDest, tonic::Status>>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    tonic::Status,
>
where
    TDest: Send + Sync + Debug + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        #[cfg(not(feature = "adjust-server-stream"))]
        let send_timeout = DEFAULT_SEND_TIMEOUT;

        let sent_result = tx
            .send_timeout(Result::<_, tonic::Status>::Ok(item), send_timeout)
            .await;

        if let Err(err) = sent_result {
            match err {
                SendTimeoutError::Timeout(err) => {
                    println!("Can not send to grpc channel. Timeout. Err: {:?}", err);
                }
                SendTimeoutError::Closed(err) => {
                    println!("Can not send to grpc channel. Its closed. Err: {:?}", err);
                }
            }
        }
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let response: Pin<
        Box<dyn futures::Stream<Item = Result<TDest, tonic::Status>> + Send + Sync + 'static>,
    > = Box::pin(output_stream);
    return Ok(tonic::Response::new(response));
}

pub async fn send_vec_to_stream<TSrc, TDest, TFn>(
    src: Vec<TSrc>,
    mapping: TFn,
    #[cfg(feature = "adjust-server-stream")] channel_size: usize,
    #[cfg(feature = "adjust-server-stream")] send_timeout: Duration,
) -> Result<
    tonic::Response<
        Pin<
            Box<
                dyn tonic::codegen::futures_core::Stream<Item = Result<TDest, tonic::Status>>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    tonic::Status,
>
where
    TSrc: Send + Sync + 'static,
    TDest: Send + Sync + Debug + 'static,
    TFn: Fn(TSrc) -> TDest + Send + Sync + 'static,
{
    #[cfg(not(feature = "adjust-server-stream"))]
    let channel_size = 100;

    let (tx, rx) = tokio::sync::mpsc::channel(channel_size);

    tokio::spawn(async move {
        #[cfg(not(feature = "adjust-server-stream"))]
        let send_timeout = DEFAULT_SEND_TIMEOUT;

        for itm in src {
            let contract = mapping(itm);

            let sent_result = tx
                .send_timeout(Result::<_, tonic::Status>::Ok(contract), send_timeout)
                .await;

            if let Err(err) = sent_result {
                match err {
                    SendTimeoutError::Timeout(err) => {
                        println!("Can not send to grpc channel. Timeout. Err: {:?}", err);
                        break;
                    }
                    SendTimeoutError::Closed(err) => {
                        println!("Can not send to grpc channel. Its closed. Err: {:?}", err);
                        break;
                    }
                }
            }
        }
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let response: Pin<
        Box<dyn futures::Stream<Item = Result<TDest, tonic::Status>> + Send + Sync + 'static>,
    > = Box::pin(output_stream);
    return Ok(tonic::Response::new(response));
}

pub async fn send_hash_map_to_stream<TKeySrc, TValueSrc, TDest, TFn>(
    src: std::collections::HashMap<TKeySrc, TValueSrc>,
    mapping: TFn,
    #[cfg(feature = "adjust-server-stream")] channel_size: usize,
    #[cfg(feature = "adjust-server-stream")] send_timeout: Duration,
) -> Result<
    tonic::Response<
        Pin<
            Box<
                dyn tonic::codegen::futures_core::Stream<Item = Result<TDest, tonic::Status>>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    tonic::Status,
>
where
    TKeySrc: Send + Sync + 'static,
    TValueSrc: Send + Sync + 'static,
    TDest: Send + Sync + Debug + 'static,
    TFn: Fn(TKeySrc, TValueSrc) -> TDest + Send + Sync + 'static,
{
    #[cfg(not(feature = "adjust-server-stream"))]
    let channel_size = 100;
    let (tx, rx) = tokio::sync::mpsc::channel(channel_size);

    tokio::spawn(async move {
        #[cfg(not(feature = "adjust-server-stream"))]
        let send_timeout = DEFAULT_SEND_TIMEOUT;
        for (key, value) in src {
            let contract = mapping(key, value);

            let sent_result = tx
                .send_timeout(Result::<_, tonic::Status>::Ok(contract), send_timeout)
                .await;

            if let Err(err) = sent_result {
                match err {
                    SendTimeoutError::Timeout(err) => {
                        println!("Can not send to grpc channel. Timeout. Err: {:?}", err);
                        break;
                    }
                    SendTimeoutError::Closed(err) => {
                        println!("Can not send to grpc channel. Its closed. Err: {:?}", err);
                        break;
                    }
                }
            }
        }
    });

    let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let response: Pin<
        Box<dyn futures::Stream<Item = Result<TDest, tonic::Status>> + Send + Sync + 'static>,
    > = Box::pin(output_stream);
    return Ok(tonic::Response::new(response));
}