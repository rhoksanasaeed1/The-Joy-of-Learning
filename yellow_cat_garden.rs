// main.rs
fn main() {
    println!("Welcome to the Joy of Learning Rust!");
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn calculate_distance(p1: &Point, p2: &Point) -> f32 {
    let delta_x = (p1.x - p2.x).pow(2);
    let delta_y = (p1.y - p2.y).pow(2);

    let distance = (delta_x + delta_y) as f32;

    return ((distance / 2.0).sqrt()) as f32;
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn len(&self) -> f32 {
        calculate_distance(&self.start, &self.end)
    }
}

struct Triangle {
    points: [Point; 3],
}

impl Triangle {
    fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { points: [p1, p2, p3] }
    }

    fn area(&self) -> f32 {
        let a = calculate_distance(&self.points[0], &self.points[1]);
        let b = calculate_distance(&self.points[1], &self.points[2]);
        let c = calculate_distance(&self.points[2], &self.points[0]);

        let s = (a + b + c) / 2.0;

        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    fn area(&self) -> i32 {
        let height = self.top_left.y - self.bottom_right.y;
        let width = self.bottom_right.x - self.top_left.x;
        return height * width;
    }
}

struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Circle {
        Circle { center, radius }
    }

    fn area(&self) -> f32 {
        return std::f32::consts::PI * (self.radius as f32).powi(2);
    }
}

enum Shape {
    Line(Line),
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Line(_) => 0.0,
            Shape::Triangle(t) => t.area(),
            Shape::Rectangle(r) => r.area() as f32,
            Shape::Circle(c) => c.area(),
        }
    }
}

struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    fn new(real: f32, imag: f32) -> Complex {
        Complex { real, imag }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

trait Printable {
    fn print(&self);
}

impl Printable for Point {
    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

struct Polygon {
    vertices: Vec<Point>,
}

impl Polygon {
    fn new(vertices: Vec<Point>) -> Polygon {
        Polygon { vertices }
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
}

impl Printable for Polygon {
    fn print(&self) {
        for vertex in self.vertices.iter() {
            vertex.print();
        }
        println!("");
    }
}

fn get_odd_numbers(nums: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for num in nums.iter() {
        if *num % 2 == 1 {
            result.push(*num);
        }
    }

    return result;
}

fn is_prime(num: i32) -> bool {
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_prime_numbers(nums: &[i32]) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();

    for num in nums.iter() {
        if is_prime(*num) {
            primes.push(*num);
        }
    }

    return primes;
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn factorial(n: i32) -> i32 {
    match n {
        0 => 1,
        n => n * factorial(n - 1),
    }
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn get_common_elements(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for x in v1.iter() {
        for y in v2.iter() {
            if *x == *y {
                result.push(*x);
            }
        }
    }

    result
}

fn bubble_sort(arr: &mut [i32]) {
    let mut length = arr.len();
    let mut is_sorted = false;

    while !is_sorted {
        is_sorted = true;
        for i in 0..(length - 1) {
            if arr[i] > arr[i + 1] {
                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
                is_sorted = false;
            }
        }
        length -= 1;
    }
}

fn matrix_multiply(mat1: &[[i32; 3]; 3], mat2: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    result
}

fn num_ways_to_make_change(coins: &[i32], num: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; (num + 1) as usize];
    dp[0] = 1;

    for &coin in coins.iter() {
        for n in 1..(num + 1) {
            if coin <= n {
                dp[n as usize] += dp[(n - coin) as usize];
            }
        }
    }

    dp[num as usize]
}

fn linear_search(arr: &[i32], num: i32) -> Option<usize> {
    for (i, &elem) in arr.iter().enumerate() {
        if elem == num {
            return Some(i);
        }
    }
    None
}

fn binary_search(arr: &[i32], num: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == num {
            return Some(mid);
        } else if arr[mid] < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}

fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid > 0 {
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        let mut tmp = vec![0; arr.len()];
        merge(&arr[..mid], &arr[mid..], &mut tmp);
        arr.copy_from_slice(&tmp);
    }
}

fn merge(left: &[i32], right: &[i32], arr: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        let mid = partition(arr);
        quick_sort(&mut arr[..mid]);
        quick_sort(&mut arr[mid + 1..]);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

fn depth_first_search(adj_list: &Vec<Vec<i32>>, start: usize) -> Vec<bool> {
    let mut visited: Vec<bool> = vec![false; adj_list.len()];
    visited[start] = true;

    let mut stack = Vec::new();
    stack.push(start);

    while !stack.is_empty() {
        let vertex = stack.pop().unwrap();
        for &neighbour in adj_list[vertex].iter() {
            if !visited[neighbour as usize] {
                visited[neighbour as usize] = true;
                stack.push(neighbour as usize);
            }
        }
    }
    visited
}

fn breadth_first_search(adj_list: &Vec<Vec<i32>>, start: usize) -> Vec<bool> {
    let mut visited: Vec<bool> = vec![false; adj_list.len()];
    visited[start] = true;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        for &neighbour in adj_list[vertex].iter() {
            if !visited[neighbour as usize] {
                visited[neighbour as usize] = true;
                queue.push_back(neighbour as usize);
            }
        }
    }
    visited
}

fn dijkstras(adj_matrix: &Vec<Vec<i32>>, start: usize) -> Vec<i32> {
    let mut dist: Vec<i32> = vec![std::i32::MAX; adj_matrix.len()];
    dist[start] = 0;

    let mut visited: Vec<bool> = vec![false; adj_matrix.len()];

    let mut index = start;
    while !visited[index] {
        visited[index] = true;
        for i in 0..adj_matrix.len() {
            if adj_matrix[index][i] > 0 && dist[i] > dist[index] + adj_matrix[index][i] {
                dist[i] = dist[index] + adj_matrix[index][i];
            }
        }

        let mut min = std::i32::MAX;
        for i in 0..adj_matrix.len() {
            if !visited[i] && dist[i] < min {
                min = dist[i];
                index = i;
            }
        }
    }
    dist
}

fn knapsack(values: &[i32], weights: &[i32], capacity: i32) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; (capacity + 1) as usize]; values.len()];

    for i in 0..values.len() {
        for w in 0..(capacity + 1) {
            if i == 0 {
                if weights[0] <= w {
                    dp[i][w as usize] = values[0];
                }
            } else if weights[i] <= w {
                dp[i][w as us