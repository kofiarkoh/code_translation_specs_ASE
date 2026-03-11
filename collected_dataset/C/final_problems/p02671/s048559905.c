#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>

int convert(int x, int n){
    int y=0, i=0, z;
    while(x > 0){
        z = x%n;
        if(z == 1){
          z = 2;
        }else if(z==2){
          z = 1;
        }
        y += z*pow(n, i);
        x = x/n;
        i++;
    }
    return y;
}

int main()
{
  int n;
  scanf("%d", &n);
  char t[200001];
  scanf("%s", t);

  int shuturyoku;
  
  for(int i = 0; i < pow(3, n); i++){
    shuturyoku = i;
    for(int j = 0; j < strlen(t); j++){
      if(t[j] == 'S'){
        shuturyoku = convert(shuturyoku, 3);
      }else if(t[j] == 'R'){
        if(shuturyoku == pow(3,n)-1){
          shuturyoku = 0;
        }else{
          shuturyoku = shuturyoku+1;
        }
      } 
      // printf("%d ", shuturyoku);
    }
    printf("%d ", shuturyoku);
    // printf("\n");
  }
  printf("\n");
  return 0;
}