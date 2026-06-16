pub fn find_rect(image: Vec<Vec<i32>>) -> Option<Rect> {
    let img = Image::new(image);

    for row in 0..img.height {
        for column in 0..img.width {
            let pixel = img.pixels[row][column];

            if pixel == 0 {
                return Some(Rect {
                    row,
                    column,
                    width: find_width(&img, row, column),
                    height: find_height(&img, row, column),
                });
            }
        }
    }

    None
}

fn find_height(image: &Image, mut row: usize, column: usize) -> usize {
    let mut count = 0;

    while is_inside_rect(image, row, column) {
        count += 1;
        row += 1;
    }

    count
}

fn find_width(image: &Image, row: usize, mut column: usize) -> usize {
    let mut count = 0;

    while is_inside_rect(image, row, column) {
        count += 1;
        column += 1;
    }

    count
}

fn is_inside_rect(image: &Image, row: usize, column: usize) -> bool {
    row < image.height && column < image.width && image.pixels[row][column] == 0
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    row: usize,
    column: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pixels: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(pixels: Vec<Vec<i32>>) -> Self {
        let height = pixels.len();
        let width = pixels.first().map(|r| r.len()).unwrap_or_default();

        Self {
            pixels,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let image = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        let expected = Rect {
            row: 2,
            column: 3,
            width: 3,
            height: 2,
        };

        assert_eq!(Some(expected), find_rect(image));

        let image = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 1, 0, 0],
        ];

        let expected = Rect {
            row: 3,
            column: 5,
            width: 2,
            height: 2,
        };

        assert_eq!(Some(expected), find_rect(image));
    }
}
