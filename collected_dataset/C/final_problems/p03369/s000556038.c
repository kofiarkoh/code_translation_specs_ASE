#include <stdio.h>
#include <stdlib.h>
#include "time.h"
#include "unistd.h"
#include "string.h"
#include <math.h>
int ans;
int main(int argc, char* argv[]){
  long int a=0,b=0,c=0;
  char d[3];
  scanf("%c%c%c",&d[0],&d[1],&d[2]);
  for (int i = 0; i < 3; i++) {
    if (d[i]=='o') {
      a++;
    }
  }
  printf("%ld\n",100*a+700);
}
