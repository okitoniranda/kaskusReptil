# kaskusReptil
# Kaskus Reptil

Aplikasi untuk pecinta reptil berbagi informasi dan jual beli.

## Fitur
- **Forum:** Diskusi dengan sesama pecinta reptil.
- **Marketplace:** Jual beli reptil, makanan, kandang, dan perlengkapan.

## Cara Menjalankan

### Backend
1. Masuk ke direktori `backend/`.
2. Bangun dan jalankan backend:
    ```bash
    cargo build --release
    cargo run
    ```

### Frontend
1. Masuk ke direktori `frontend/`.
2. Bangun frontend dengan WebAssembly:
    ```bash
    wasm-pack build --target web
    ```
3. Jalankan server lokal:
    ```bash
    python3 -m http.server 8080
    ```

Akses di `http://localhost:8080`.
