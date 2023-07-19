//This program prints 'The Joy of Learning'

fun main(args: Array<String>) {
    println("The Joy of Learning")
}

//Class to represent a Book
class Book {
    var title: String = ""
    var author: String = ""
    var publisher: String = ""
    var year: Int = 0
    
    //Function to read a book
    fun read() {
        println("You are now reading the book: $title")
    }
    
    //Function to write a review for a book
    fun review() {
        println("You are now writing a review for the book: $title")
    }
    
    //Function to locate a book
    fun find() {
        println("Searching for book: $title")
    }
}

//Class to represent a LearningExperience
class LearningExperience {
    var title: String = ""
    var teacher: String = ""
    var subject: String = ""
    var duration: Int = 0
    
    //Function to attend the lecture
    fun attend() {
        println("You are now attending the lecture: $title")
    }
    
    //Function to take notes
    fun notes() {
        println("You are now taking notes for the lecture: $title")
    }
    
    //Function to locate a lecture
    fun find() {
        println("Searching for lecture: $title")
    }
}

//Function to print out welcome message
fun intro() {
    println("Welcome to The Joy of Learning")
}

//Function to print out closing message
fun outro() {
    println("Thanks for learning with us!")
}

//Function to add a book to the library
fun addBook(title: String, author: String, publisher: String, year: Int) {
    val book = Book()
    book.title = title
    book.author = author
    book.publisher = publisher
    book.year = year
    
    println("Added book: $title")
}

//Function to add a learning experience to the library
fun addExperience(title: String, teacher: String, subject: String, duration: Int) {
    val experience = LearningExperience()
    experience.title = title
    experience.teacher = teacher
    experience.subject = subject
    experience.duration = duration
    
    println("Added lecture: $title")
} 

//Function to read a book from the library
fun readBook(title: String) {
    val book = Book()
    book.title = title
    book.read()
}

//Function to attend a lecture from the library
fun attendExperience(title: String) {
    val experience = LearningExperience()
    experience.title = title
    experience.attend()
}

//Function to write a review for a book from the library
fun reviewBook(title: String) {
    val book = Book()
    book.title = title
    book.review()
    
    println("Thanks for your review!")
}

//Function to take notes at a lecture from the library
fun notesExperience(title: String) {
    val experience = LearningExperience()
    experience.title = title
    experience.notes()
}

//Function to search for a book from the library
fun findBook(title: String) {
    val book = Book()
    book.title = title
    book.find()
}

//Function to search for a lecture from the library
fun findExperience(title: String) {
    val experience = LearningExperience()
    experience.title = title
    experience.find()
}