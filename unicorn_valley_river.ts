//1
// Class to define a student
class Student {
    constructor(name, age) {
      this.name = name;
      this.age = age;
    }
  
    getName() {
      return this.name;
    }
  
    getAge() {
      return this.age;
    }
  }
  
  //2
  // Function to create an instance of the student  
  function createStudent(name, age) {
    return new Student(name, age);
  }
  
  //3
  // Class to define a course
  class Course {
    constructor(name, duration, difficulty) {
      this.name = name;
      this.duration = duration;
      this.difficulty = difficulty;
    }
  
    getName() {
      return this.name;
    }
  
    getDuration() {
      return this.duration;
    }
  
    getDifficulty() {
      return this.difficulty;
    }
  }
  
  //4
  // Function to create an instance of the course  
  function createCourse(name, duration, difficulty) {
    return new Course(name, duration, difficulty);
  }
  
  //5
  // Class to define a student-course relationship  
  class StudentCourse{
    constructor(student, course) {
      this.student = student;
      this.course = course;
    }
  
    getStudentName() {
      return this.student.name;
    }
  
    getCourseName() {
      return this.course.name;
    }
  }
  
  //6
  // Function to create an instance of the student-course relationship 
  function createStudentCourse(student, course) {
    return new StudentCourse(student, course);
  }
  
  //7
  // Function to find out the name of the student associated with a course  
  function getStudentName(studentCourse) {
    return studentCourse.getStudentName();
  }
  
  //8
  // Function to find out the name of the course associated with a student  
  function getCourseName(studentCourse) {
    return studentCourse.getCourseName();
  }
  
  //9
  // Function to enrol a student in a course  
  function enrolStudent(name, age, courseName, duration, difficulty) {
    const student = createStudent(name, age);
    const course = createCourse(courseName, duration, difficulty);
    return createStudentCourse(student, course);
  }
  
  //10
  // Function to print out the details of a student and course
  function printStudentCourseDetails(studentCourse) {
    console.log(`${studentCourse.getStudentName()} is taking the ${studentCourse.getCourseName()} course`);
  }
  
  //11
  // Function to get the total number of enrolled students
  function getTotalEnrolledStudents() {
    return enrolledStudents.length;
  }
  
  //12
  // Variable to store enrolled students
  let enrolledStudents = [];
  
  //13
  // Function to add an enrolled student to the list
  function addToEnrolledStudents(studentCourse) {
    enrolledStudents.push(studentCourse);
  }
  
  //14
  // Function to remove an enrolled student from the list
  function removeFromEnrolledStudents(index) {
    enrolledStudents.splice(index, 1);
  }
  
  //15
  // Function to update the details of an enrolled student
  function updateEnrolledStudent(index, studentCourse) {
    enrolledStudents[index] = studentCourse;
  }
  
  //16
  // Function to print out the details of all enrolled students
  function printEnrolledStudentsDetails() {
    enrolledStudents.forEach((studentCourse) => {
      printStudentCourseDetails(studentCourse);
    });
  }
  
  //17
  // Function to get the average age of enrolled students
  function getAverageAge() {
    let totalAge = 0;
    let avgAge;
  
    enrolledStudents.forEach((studentCourse) => {
      totalAge += studentCourse.student.age;
    });
  
    avgAge = totalAge / enrolledStudents.length;
    return avgAge;
  }
  
  //18
  // Function to get the duration of the longest course
  function getLongestCourseDuration() {
    let longestDuration = 0;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.duration > longestDuration) {
        longestDuration = studentCourse.course.duration;
      }
    });
  
    return longestDuration;
  }
  
  //19
  // Function to get the average difficulty of the courses
  function getAverageDifficulty() {
    let totalDifficulty = 0;
    let avgDifficulty;
  
    enrolledStudents.forEach((studentCourse) => {
      totalDifficulty += studentCourse.course.difficulty;
    });
  
    avgDifficulty = totalDifficulty / enrolledStudents.length;
    return avgDifficulty;
  }
  
  //20
  // Function to print out the details of the student taking the longest course
  function printLongestCourseStudent() {
    let longestDuration = getLongestCourseDuration();
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.duration === longestDuration) {
        console.log(`The student taking the longest course is ${studentCourse.student.name}`);
      }
    });
  }
  
  //21
  // Function to print out the details of the student with the highest difficulty course
  function printHighestDifficultyStudent() {
    let highestDifficulty = 0;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.difficulty > highestDifficulty) {
        highestDifficulty = studentCourse.course.difficulty;
      }
    });
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.difficulty === highestDifficulty) {
        console.log(`The student taking the highest difficulty course is ${studentCourse.student.name}`);
      }
    });
  }
  
  //22
  // Function to get the total number of courses enrolled
  function getTotalCoursesEnrolled() {
    const courses = enrolledStudents.map((studentCourse) => studentCourse.course.name);
    return [...new Set(courses)].length;
  }
  
  //23
  // Function to print out the names of all students enrolled in a course
  function printStudentsEnrolledInCourse(courseName) {
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.name === courseName) {
        console.log(studentCourse.student.name);
      }
    });
  }
  
  //24
  // Function to print out the names of all courses enrolled by a student
  function printCoursesEnrolledByStudent(studentName) {
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.student.name === studentName) {
        console.log(studentCourse.course.name);
      }
    });
  }
  
  //25
  // Function to get the total number of students enrolled in a course
  function getTotalStudentsEnrolledInCourse(courseName) {
    let totalStudents = 0;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.name === courseName) {
        totalStudents++;
      }
    });
  
    return totalStudents;
  }
  
  //26
  // Function to get the total number of courses enrolled by a student
  function getTotalCoursesEnrolledByStudent(studentName) {
    let totalCourses = 0;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.student.name === studentName) {
        totalCourses++;
      }
    });
  
    return totalCourses;
  }
  
  //27
  // Function to get the total number of courses grouped by difficulty
  function getTotalCoursesByDifficulty() {
    const courseDifficulties = enrolledStudents.map((studentCourse) => studentCourse.course.difficulty);
  
    const difficultyCounts = {
      easy: 0,
      medium: 0,
      hard: 0,
    };
  
    courseDifficulties.forEach((difficulty) => {
      switch (difficulty) {
        case 1:
          difficultyCounts.easy++;
          break;
  
        case 2:
          difficultyCounts.medium++;
          break;
  
        case 3:
          difficultyCounts.hard++;
          break;
      }
    });
  
    return difficultyCounts;
  }
  
  //28
  // Function to get the total number of enrolled students grouped by age
  function getTotalEnrolledStudentsByAge() {
    const studentAges = enrolledStudents.map((studentCourse) => studentCourse.student.age);
  
    const ageCounts = {
      under18: 0,
      18to25: 0,
      above25: 0,
    };
  
    studentAges.forEach((age) => {
      if (age < 18) {
        ageCounts.under18++;
      } else if (age >= 18 && age <= 25) {
        ageCounts.18to25++;
      } else {
        ageCounts.above25++;
      }
    });
  
    return ageCounts;
  }
  
  //29
  // Function to find a student by name
  function findStudent(name) {
    let student = null;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.student.name === name) {
        student = studentCourse.student;
      }
    });
  
    return student;
  }
  
  //30
  // Function to find a course by name
  function findCourse(name) {
    let course = null;
  
    enrolledStudents.forEach((studentCourse) => {
      if (studentCourse.course.name === name) {
        course = studentCourse.course;
      }
    });
  
    return course;
  }
  
  //31
  // Function to find a student-course relationship by student name
  function findStudentCourseByStudentName(name) {
    let studentCourse = null;
  
    enrolledStudents.forEach((enrolledStudent) => {
      if (enrolledStudent.student.name === name) {
        studentCourse = enrolledStudent;
      }
    });
  
    return studentCourse;
  }
  
  //32
  // Function to find a student-course relationship by course name
  function findStudentCourseByCourseName(name) {
    let studentCourse = null;
  
    enrolledStudents.forEach((enrolledStudent) => {
      if (enrolledStudent.course.name === name) {
        studentCourse = enrolledStudent;
      }
    });
  
    return studentCourse;
  }
  
  //33
  // Class to define a learning session
  class LearningSession {
    constructor(time, duration, student, course) {
      this.time = time;
      this.duration = duration;
      this.student = student;
      this.course = course;
    }
  
    getTime() {
      return this.time;
    }
  
    getDuration() {
      return this.duration;
    }
  
    getStudentName() {
      return this.student.name;
    }
  
    getCourseName() {
      return this.course.name;
    }
  }
  
  //34
  // Function to create an instance of the learning session
  function createLearningSession(time, duration, student, course) {
    return new LearningSession(time, duration, student, course);
  }
  
  //35
  // Variable to store learning sessions
  let learningSessions = [];
  
  //36
  // Function to add a learning session to the list
  function addToLearningSessions(learningSession) {
    learningSessions.push(learningSession);
  }
  
  //37
  // Function to remove a learning session from the list
  function removeFromLearningSessions(index) {
    learningSessions.splice(index, 1);
  }
  
  //38
  // Function to update the details of a learning session
  function updateLearningSession(index, learningSession) {
    learningSessions[index] = learningSession;
  }
  
  //39
  // Function to print out the details of a learning session
  function printLearningSessionDetails(learningSession) {
    console.log(`${learningSession.getStudentName()} is learning ${learningSession.getCourseName()} at ${learningSession.getTime()} for ${learningSession.getDuration()} hours`);
  }
  
  //40
  // Function to print out the details of all learning sessions
  function printLearningSessionsDetails() {
    learningSessions.forEach((learningSession) => {
      printLearningSessionDetails(learningSession);
    });
  }
  
  //41
  // Function to get the total number of learning sessions added
  function getTotalLearningSessions() {
    return learningSessions.length;
  }
  
  //42
  // Function to get the total hours spent learning by a student
  function getTotalHoursSpentLearningByStudent(name) {
    let totalHours = 0;
  
    learningSessions.forEach((learningSession) => {
      if (learningSession.student.name === name) {
        totalHours += learningSession.duration;
      }
    });
  
    return totalHours;
  }
  
  //43
  // Function to get the total hours spent learning by a course
  function getTotalHoursSpentLearningByCourse(name) {
    let totalHours = 0;
  
    learningSessions.forEach((learningSession) => {
      if (learningSession.course.name === name) {
        totalHours += learningSession.duration;
      }
    });
  
    return totalHours;
  }
  
  //44
  // Function to find a learning session by student name
  function findLearningSessionByStudentName(name) {
    let learningSession = null;
  
    learningSessions.forEach((session) => {
      if (session.student.name === name) {
        learningSession = session;
      }
    });
  
    return learningSession;
  }
  
  //45
  // Function to find a learning session by course name