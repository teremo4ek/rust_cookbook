use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    // Создаем дескриптор файла
    let tar_gz = File::create("archive.tar.gz")?;
    // Создаем экземпляр "упаковщика" архива, передавая в конструктор
    // дескриптор файла и метод сжатия
    let enc = GzEncoder::new(tar_gz, Compression::default());
    // Создаем дескриптор архива
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

// use flate2::read::GzDecoder;
// use std::fs::File;
// use tar::Archive;

// fn main() -> Result<(), std::io::Error> {
//     // Название архива (путь к нему)
//     let path = "archive.tar.gz";
//     // Открываем файл (создаем дескриптор файла)
//     let tar_gz = File::open(path)?;
//     // Создаем экземпляр "распаковщика" архива, передавая в конструктор дескриптор файла
//     let tar = GzDecoder::new(tar_gz);
//     // Создаем экземпляр дескриптора архива
//     let mut archive = Archive::new(tar);
//     // Извлекаем файлы из архива в текущую директорию
//     archive.unpack(".")?;

//     Ok(())
// }
