#include <iostream>
#include <string>

using namespace std;

int main()
{
	//initialize variables
	string joy = "The Joy of Learning";
	int count = 0;

	//loop through the string and print one letter at a time
	while (count < joy.length())
	{
		cout << joy[count];
		count++;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every other letter starting from the beginning
	count = 0;
	while (count < joy.length()) 
	{
		cout << joy[count];
		count = count + 2;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every second letter starting from the second last letter
	count = joy.length() - 1;
	while (count >= 0) 
	{
		cout << joy[count];
		count = count - 2;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every third letter starting from the last letter
	count = joy.length() - 1;
	while (count >= 0) 
	{
		cout << joy[count];
		count = count - 3;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every fourth letter starting from the first letter
	count = 0;
	while (count < joy.length()) 
	{
		cout << joy[count];
		count = count + 4;
	} 

	//enter a line break
	cout << endl;

	//loop through the string and print every fifth letter starting from the fourth letter
	count = 3;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 5;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every sixth letter starting from the third letter
	count = 2;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 6;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every seventh letter starting from the second letter
	count = 1;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 7;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every eighth letter starting from the first letter
	count = 0;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 8;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every ninth letter starting from the eighth letter
	count = 7;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 9;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every tenth letter starting from the seventh letter
	count = 6;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 10;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every eleventh letter starting from the sixth letter
	count = 5;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 11;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every twelfth letter starting from the fifth letter
	count = 4;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 12;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every thirteenth letter starting from the fourth letter
	count = 3;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 13;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every fourteenth letter starting from the third letter
	count = 2;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 14;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every fifteenth letter starting from the second letter
	count = 1;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 15;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every sixteenth letter starting from the first letter
	count = 0;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 16;
	}

	//enter a line break
	cout << endl;

	//loop through the string and print every seventeenth letter starting from the sixteenth letter
	count = 15;
	while (count < joy.length())
	{
		cout << joy[count];
		count = count + 17;
	}

	//enter a line break
	cout << endl; 

	//return statement
	return 0;
}