# Final Project: Automation Testing with Katalon Studio
 
Proyek ini adalah solusi automasi testing menggunakan **Katalon Studio** untuk tiga platform: **Android**, **Website**, dan **API**. Fokus utama proyek ini adalah mengotomatisasi pengujian fitur-fitur kritis seperti **Login**, **Register**, **Buy Event**, dan **Update Profile** pada website dan aplikasi `coding.id`.

## Fitur yang Diuji

- **Login**: Menguji alur login dengan kredensial valid dan invalid.
- **Register**: Menguji pendaftaran pengguna baru.
- **Buy Event**: Menguji proses pembelian event atau kursus.
- **Update Profile**: Menguji pembaruan informasi profil pengguna.

## Platform yang Didukung

1. **Android**: Automasi testing untuk aplikasi mobile `coding.id`.
2. **Website**: Automasi testing untuk website `coding.id`.
3. **API**: Automasi testing untuk API yang digunakan oleh `coding.id`.

## Teknologi yang Digunakan

- **Katalon Studio**: Tools utama untuk automasi testing.
- **Groovy**: Bahasa scripting yang digunakan di Katalon Studio.
- **Appium**: Untuk automasi testing aplikasi Android.
- **Postman/API Testing Tools**: Untuk pengujian API (jika diperlukan).

## Struktur Proyek

```
finalProject/
├── Test Cases/               # Berisi test case untuk setiap fitur
│   ├── Android/              # Test case untuk platform Android
│   ├── Web/                  # Test case untuk platform Website
│   └── API/                  # Test case untuk platform API
├── Object Repository/        # Lokasi penyimpanan elemen UI yang diidentifikasi
│   ├── Android/              # Object repository untuk Android
│   └── Web/                  # Object repository untuk Website
│   └── API/                  # Object repository untuk Website
├── Test Suites/              # Kumpulan test case yang diorganisir dalam test suite
├── Scripts/                  # Script Groovy kustom
├── Data Files/               # File data yang digunakan untuk testing (contoh: CSV, Excel)
├── Reports/                  # Laporan hasil testing
└── README.md                 # File README
```

## Prasyarat

Sebelum menjalankan proyek ini, pastikan Anda telah menginstal:

- **Katalon Studio**: [Download Katalon Studio](https://katalon.com/download/).
- **Android SDK**: Untuk testing aplikasi Android.
- **Web Browser**: Chrome, Firefox, atau browser lain yang didukung.
- **Appium**: Jika melakukan testing mobile
- **Java JDK**: Katalon Studio membutuhkan Java JDK.

## Cara Menjalankan Proyek

1. **Clone repositori**:
   ```bash
   git clone https://github.com/esyeer/finalProject.git
   cd finalProject
   ```

2. **Buka proyek di Katalon Studio**:
   - Buka Katalon Studio.
   - Pilih **Open Project** dan arahkan ke folder `finalProject`.

3. **Jalankan Test Suite**:
   - Buka folder `Test Suites`.
   - Pilih test suite yang ingin dijalankan (Android, Web, atau API).
   - Klik **Run** untuk menjalankan test suite.

4. **Lihat Hasil Testing**:
   - Setelah selesai, hasil testing akan tersedia di folder `Reports`.

## Contoh Test Case

### Login (Website)
1. Buka halaman login `coding.id`.
2. Input email dan password yang valid.
3. Klik tombol login.
4. Verifikasi bahwa pengguna berhasil login.

### Buy Event (Android)
1. Buka aplikasi `coding.id`.
2. Navigasi ke halaman event.
3. Pilih event yang tersedia.
4. Klik tombol "Buy".
5. Verifikasi bahwa event berhasil dibeli.

### Update Profile (API)
1. Kirim request API untuk login dan dapatkan token.
2. Gunakan token untuk mengirim request update profile.
3. Verifikasi response API untuk memastikan profile berhasil diupdate.

## Kontribusi

Kontribusi sangat diterima! Jika Anda ingin berkontribusi pada proyek ini, silakan ikuti langkah-langkah berikut:

1. Fork repositori ini.
2. Buat branch baru (`git checkout -b fitur-baru`).
3. Commit perubahan Anda (`git commit -m 'Menambahkan fitur baru'`).
4. Push ke branch (`git push origin fitur-baru`).
5. Buat Pull Request.


## Kontak

Jika Anda memiliki pertanyaan atau masukan, silakan hubungi:

- **Nama Anda**: [Syarif Hidayatullah]  
- **GitHub**: [esyeer](https://github.com/esyeer)  

---
