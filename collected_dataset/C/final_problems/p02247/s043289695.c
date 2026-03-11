#include <stdio.h>
#include <string.h>
 
int same(char *T, char *P, int i, int plength){
  int j;
 
  for(j = 0; j < plength; j++){
    if(T[i+j] != P[j]){
      return 0;
    }
  }
  return -1;
}
 
int main(){
  char T[1000], P[1000];
  int i, tlength, plength;
 
  scanf("%s", T);
  scanf("%s", P);
 
  tlength = strlen(T);
  plength = strlen(P);
 
  for(i = 0; i < tlength; i++){
    if(same(T, P, i, plength)){
      printf("%d\n", i);
    }     
  }
  return 0;
}