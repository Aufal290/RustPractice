fn main() {
    let daftar_harga = [15000, 20000, 50000];

    let barang_spesial = ("Keyboard Mechanical", 450000, 0);

    println!("--- Selamat Datang di sini ---");

    println!("Daftar Harga di Katalog:");
    for harga in daftar_harga {
        if harga > 25000
        {
            println!("harga mahal");
        }
        println!("Rp {harga}");
    }

    let (nama, harga, stok) = barang_spesial;
    println!("\nBarang Spesial: {nama}");
    println!("Harga: Rp {harga}, Stok: {stok}");

   if data_boolean(stok)
   {
    println!("Stok tersedia");
   }
   else
   {
    println!("Stok Tidak tersedia");
   }

    let harga_diskon = hitung_diskon(harga, 10); 
    println!("Harga setelah diskon 10%: Rp {harga_diskon}");
}

fn hitung_diskon(harga: i32, persen: i32) -> i32 {

    harga - (harga * persen / 100)
}

fn data_boolean(stok: i32) -> bool
{
   stok > 0
}