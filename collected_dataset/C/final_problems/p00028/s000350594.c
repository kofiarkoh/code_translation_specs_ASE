#include <stdio.h>
#include <string.h>

int main()
{
  int countArray[101], number, mx_number = 0;
  memset(countArray, 0, sizeof countArray);

  while(scanf("%d", &number)!=EOF){
    ++countArray[number];
  }

  for(number = 100; number > 0; --number)
    if(countArray[number] > countArray[mx_number])
      mx_number = number;

  for(number = 1; number <= mx_number; ++number)
    if(countArray[number] == countArray[mx_number])
      printf("%d\n", number);

  return 0;
}