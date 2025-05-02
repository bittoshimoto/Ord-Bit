
# B1T Ordinals Indexer MAY STILL HAVE BUGS

ℹ️ This is a fork adapted for the B1T blockchain from [apezord/ord-dogecoin](https://github.com/apezord/ord-dogecoin).

## **Prerequisites**
To run the **B1T Ordinals Indexer**, you must set up and fully sync a **B1T** node.

### 1. Install B1T Core  
Download and install the latest version from [B1T Core](https://github.com/bittoshimoto/Bit).

### 2. Start Your B1T Node  
Run the following command to start **B1T Core** with the required flags:

```shell
bitd -txindex -rpcuser=your_username -rpcpassword=your_password -rpcport=42069 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
```

- Ensure your **B1T node is fully synced** before starting the indexer.
- ‼️ **IMPORTANT**: Replace `your_username` and `your_password` with secure credentials.

---

## **Building the Indexer**

### 1. Install Dependencies  
Ensure that you have the necessary dependencies installed:

```shell
sudo apt update && sudo apt install -y build-essential clang pkg-config libssl-dev git
```

### 2. Install Rust  
If you don't have **Rust** installed, install it using the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Clone the Repository  
```shell
git clone https://github.com/bittoshimoto/Ord-Bit
cd Ord-Bit
```

### 4. Build the Release Version  
```shell
cargo build --release
```

---

## **Running the Indexer**

### **1. Ensure the Data Directory Exists**
```shell
mkdir -p /mnt/ord-node/indexer-data-main
```

### **2. Start Indexing**
Replace `YOUR_RPC_URL` with your actual **B1T node URL** (e.g., `http://your_username:your_password@127.0.0.1:42069`).

```shell
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://your_username:your_password@127.0.0.1:42069 \
    --data-dir=/mnt/ord-node/indexer-data-main \
    --index-transactions \
    --index-dunes \
    --index-bit20 \
    --nr-parallel-requests=16 \
    index
```

### **3. Start the Indexer with the Server**
```shell
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://your_username:your_password@127.0.0.1:42069 \
    --data-dir=/mnt/ord-node/indexer-data-main \
    --index-transactions \
    --index-dunes \
    --index-bit20 \
    --nr-parallel-requests=16 \
    server --http-port 8080
```

---

## **Important Parameters**
- `--index-transactions`: Stores transaction data for better API performance.
- `--index-bit20`: Enables indexing of **BIT-20 tokens, including Dev-20’s** *(subject to change)*.
- `--index-dunes`: Enables indexing of **Dunes**.
- `--nr-parallel-requests=16`: Configures parallel requests to your RPC Server.
- `--data-dir`: Specifies where the indexer stores its data.
- `--http-port`: The port where the server will listen (**default: 8080**).

---

## **Storage Requirements**
The database size depends on the indexing options enabled and the current blockchain size. Ensure you have at least **400GB of free storage**.

---

## **API Documentation**
You can find the API documentation in the [openapi.yaml](https://github.com/bittoshimoto/Ord-Bit/blob/main/openapi.yaml) file.  
The most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).

---

## **Troubleshooting**

### **Indexer Not Syncing?**
- Ensure that **B1T Core** is running and fully synced.
- Confirm that `-txindex` is enabled in your **B1T** node.
- Check that `rpcuser` and `rpcpassword` match in both `bitd` and the indexer command.

### **Out of Disk Space?**
- You need at least **400GB** of free space for the full index.

### **Indexer Crashing?**
- Run the command with `RUST_BACKTRACE=1` for more debugging information:
  ```shell
  RUST_BACKTRACE=1 ./target/release/ord --rpc-url=http://your_username:your_password@127.0.0.1:42069 --data-dir=/mnt/ord-node/indexer-data-main index
  ```

---

## **Contributing**
Contributions are welcome! Please feel free to submit a **Pull Request**.  
For major changes, open an **Issue** first to discuss the modifications.

---

## **License**
This project is licensed under [CC0-1.0 license](https://github.com/bittoshimoto/Ord-Bit/blob/main/LICENSE).

---

## **Repository**
For more information, source code, and updates, visit the [GitHub repository](https://github.com/bittoshimoto/Ord-Bit).
```

