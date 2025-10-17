fn main() {
  let mut res = winres::WindowsResource::new();
  res.set("ProductName", "getmacaddress");
  res.set("FileDescription", "A lightweight utility for retrieving the hostname and MAC address of Wi-Fi (IEEE 802.11) network interfaces on Windows.");
  res.set("CompanyName", "PT. Indonesia Epson Industry");
  res.set("LegalCopyright", "© 2025 PT. Indonesia Epson Industry");
  res.set("OriginalFilename", "getmacaddress.exe");
  res.set("FileVersion", "0.1.1.0");
  res.set("ProductVersion", "0.1.1.0");
  res.compile().expect("Failed to compile resources");
}