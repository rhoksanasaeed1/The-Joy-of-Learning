<?php

// Part 1 
// Create a simple function to prints a welcome message

function printWelcomeMessage(){
	// Create a variable to store the welcome message
	$message = "Welcome to the Joy of Learning!";
	
	// Print the welcome message
	echo $message;
}

// Part 2
// Create a function to print a personalized goodbye message

function printGoodbyeMessage($name){
	// Create a variable to store the goodbye message
	$message = "Goodbye $name. Thanks for learning with us!";
	
	// Print the goodbye message
	echo $message;
}

// Part 3
// Create a function to create an array of the alphabet

function createAlphabet(){
	// Create an array to store the alphabet
	$alphabet = array('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
	
	// Return the array
	return $alphabet;
}

// Part 4
// Create a function to count how many elements an array has

function countElements($array){
	// Count the number of elements in the array
	$count = count($array);
	
	// Return the count
	return $count;
}

// Part 5
// Create a function to reverse an array

function reverseArray($array){
	// Reverse the array
	$reversedArray = array_reverse($array);
	
	// Return the reversed array
	return $reversedArray;
}

// Part 6
// Create a function to print out the contents of an array

function printArray($array){
	// Loop through each element in the array
	foreach ($array as $element) {
		// Print each element
		echo $element . " ";
	}
	
	// Print a new line
	echo "\n";
}

// Part 7
// Create a function to remove a specified element from an array

function removeElement($array, $elementToRemove){
	// Check if the element exists in the array
	if(in_array($elementToRemove, $array)){
		// Remove the element from the array
		$newArray = array_diff($array, array($elementToRemove));
		
		// Return the new array
		return $newArray;
	} else {
		// Return the unchanged array
		return $array;
	}
}

// Part 8
// Create a function to print out each letter of the alphabet with its corresponding number

function printAlphabetWithNumbers(){
	// Create an array for the alphabet
	$alphabet = createAlphabet();
	
	// Loop through each element in the array
	foreach ($alphabet as $number=>$letter) {
		// Print each letter with its corresponding number
		echo $number . ": " . $letter . "\n";
	}
}

// Part 9
// Create a function to calculate the mean of an array

function calculateMean($array){
	// Calculate the mean value
	$mean = array_sum($array) / countElements($array);
	
	// Return the mean
	return $mean;
}

// Part 10
// Create a function to calculate the median of an array

function calculateMedian($array){
	// Sort the array
	sort($array);
	
	// Calculate the median
	$count = countElements($array);
	$middleIndex = floor(($count-1)/2);
	if($count % 2) {
		$median = $array[$middleIndex];
	} else {
		$low = $array[$middleIndex];
		$high = $array[$middleIndex+1];
		$median = (($low + $high) / 2);
	}
	
	// Return the median
	return $median;
}

// Part 11
// Create a function to calculate the mode of an array

function calculateMode($array){
	// Create an array to store the counts of each element
	$counts = array_count_values($array);
	
	// Get the highest count value
	$maxCount = max($counts);
	
	// Find the elements that have the highest count
	$modes = array_keys($counts, $maxCount);
	
	// Return the modes
	return $modes;
}

// Part 12
// Create a function to calculate the variance of an array

function calculateVariance($array){
	// Get the mean of the array
	$mean = calculateMean($array);
	
	// Calculate the total sum of squares
	$sumOfSquares = 0;
	foreach ($array as $value) {
        $sumOfSquares += pow(($value - $mean), 2);
    }
	
	// Calculate the variance
	$variance = $sumOfSquares / countElements($array);
	
	// Return the variance
	return $variance;
}

// Part 13
// Create a function to calculate the standard deviation of an array

function calculateStandardDeviation($array){
	// Calculate the variance of the array
	$variance = calculateVariance($array);
	
	// Calculate the standard deviation
	$standardDeviation = sqrt($variance);
	
	// Return the standard deviation
	return $standardDeviation;
}

// Part 14
// Create a function to find the maximum value in an array

function findMax($array){
	// Find the maximum value
	$max = max($array);
	
	// Return the maximum value
	return $max;
}

// Part 15
// Create a function to find the minimum value in an array

function findMin($array){
	// Find the minimum value
	$min = min($array);
	
	// Return the minimum value
	return $min;
}

// Part 16
// Create a function to create an array of random numbers

function createRandomNumberArray($size){
	// Create an array to store the random numbers
	$randomNumberArray = array();
	
	// Loop through the size of the array
	for($i = 0; $i < $size; $i++){
		// Generate a random number
		$randomNumber = rand(1, 10);
		
		// Add the random number to the array
		array_push($randomNumberArray, $randomNumber);
	}
	
	// Return the array
	return $randomNumberArray;
}

// Part 17
// Create a function to shift the elements of an array one position to the left

function shiftLeft($array){
	// Create a variable to store the first element
	$firstElement = array_shift($array);
	
	// Add the first element to the end of the array
	array_push($array, $firstElement);
	
	// Return the shifted array
	return $array;
}

// Part 18
// Create a function to shift the elements of an array one position to the right

function shiftRight($array){
	// Create a variable to store the last element
	$lastElement = array_pop($array);
	
	// Add the last element to the start of the array
	array_unshift($array, $lastElement);
	
	// Return the shifted array
	return $array;
}

// Part 19
// Create a function to reverse the order of elements in an array

function reverseOrder($array){
	// Reverse the order of the elements in the array
	$reversedArray = array_reverse($array);
	
	// Return the reversed array
	return $reversedArray;
}

// Part 20
// Create a function to calculate the factorial of a number

function calculateFactorial($number){
	// Create a variable to store the factorial
	$factorial = 1;
	
	// Calculate the factorial
	for($i = 1; $i <= $number; $i++) {
		$factorial *= $i;
	}
	
	// Return the factorial
	return $factorial;
}

// Part 21
// Create a function to find the first common element in two arrays

function findFirstCommonElement($array1, $array2){
	// Loop through the first array
	foreach ($array1 as $element) {
		// Check if the element exists in the second array
		if(in_array($element, $array2)){
			// Return the element if it is found
			return $element;
		}
	}
	
	// Return null if no common element is found
	return null;
}

// Part 22
// Create a function to check if a number is prime

function isPrime($number){
	// Check if the number is less than 2
	if($number < 2){
		return false;
	}
	
	// Check if the number is divisible by any number from 2 to the number itself
	for($i = 2; $i <= $number/2; $i++) {
        if($number % $i == 0) {
            return false;
        }
    }
	
	// Return true if it is a prime number
	return true;
}

// Part 23
// Create a function to find the greatest common divisor of two numbers

function findGCD($number1, $number2){
	// Check if either of the numbers is zero
	if ($number1 == 0 || $number2 == 0){
		return 0;
	}
	
	// Calculate the GCD
	for($i = min($number1, $number2); $i > 0; $i-=1){
		if (($number1 % $i == 0) && ($number2 % $i == 0)){
			return $i;
		}
	}
}

// Part 24
// Create a function to calculate the Fibonacci number of a given index

function calculateFibonacciNumber($index){
	// Initialize the first two numbers
	$first = 0;
	$second = 1;
	
	// Calculate the Fibonacci number of the given index
	for($i = 0; $i < $index; $i++){
		$third = $first + $second;
		$first = $second;
		$second = $third;
	}
	
	// Return the Fibonacci number
	return $third;
}

// Part 25
// Create a function to calculate the sum of the first n natural numbers

function calculateNaturalNumberSum($n){
	// Calculate the sum of the first n natural numbers
	$sum = ($n * ($n + 1)) / 2;
	
	// Return the sum
	return $sum;
}

// Part 26
// Create a function to check if a number is palindrome

function isPalindrome($number){
	// Create a variable to store the reversed number
	$reversedNumber = 0;
	
	// Reverse the number
	$tempNumber = $number;
	while($tempNumber > 0){
		$digit = $tempNumber % 10;
		$tempNumber = floor($tempNumber / 10);
		$reversedNumber = ($reversedNumber * 10) + $digit;
	}
	
	// Check if the reversed number is equal to the original number
	if($number == $reversedNumber){
		return true;
	} else {
		return false;
	}
}

// Part 27
// Create a function to find the smallest multiple of two numbers

function findSmallestMultiple($number1, $number2){
	// Create a variable to store the multiple
	$multiple = 0;
	
	// Find the smallest multiple
	$i = 1;
	while(true){
		$multiple = $number1 * $number2 * $i;
		
		if (($multiple % $number1 == 0) && ($multiple % $number2 == 0)){
			break;
		}
		
		$i++;
	}
	
	// Return the multiple
	return $multiple;
}

// Part 28
// Create a function to find the largest common divisor of two numbers

function findLCD($number1, $number2){
	// Create a variable to store the largest common divisor
	$lcd = 1;
	
	// Find the smallest of the two numbers
	$smaller = min($number1, $number2);
	
	// Loop through the smaller of the two numbers
	for($i = $smaller; $i > 0; $i--){
		// Check if the number is a common divisor
		if(($number1 % $i == 0) && ($number2 % $i == 0)){
			$lcd = $i;
			break;
		}
	}
	
	// Return the largest common divisor
	return $lcd;
}

// Part 29
// Create a function to find the largest prime factor of a number

function findLargestPrimeFactor($number){
	// Find the largest prime factor
	$i = 2;
	while($i <= $number){
		if(($number % $i == 0) && isPrime($i)){
			$number = $number / $i;
		}
		$i++;
	}
	
	// Return the largest prime factor
	return $i;
}

// Part 30
// Create a function to find the total number of divisors of a number

function countDivisors($number){
	// Create a variable to store the count of divisors
	$count = 0;
	
	// Loop through the number
	for($i = 1; $i <= $number; $i++){
		if($number % $i == 0){
			$count++;
		}
	}
	
	// Return the count of divisors
	return $count;
}

// Part 31
// Create a function to sum the digits of a number

function sumDigits($number){
	// Create a variable to store the sum
	$sum = 0;
	
	// Loop through each digit
	while($number > 0){
		$sum += $number % 10;
		$number = floor($number / 10);
	}
	
	// Return the sum
	return $sum;
}

// Part 32
// Create a function to generate a random string

function generateRandomString($length){
	// Create a string to