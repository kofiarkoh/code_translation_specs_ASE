#include <stdio.h>

int main(){
  int n;
  scanf("%d\n", &n);
  char s[n];
  scanf("%s", &s);
  
  int white = 0;
  int black = 0;
  
  for(int i = 0; i < n; i++){
    if(s[i] != '.')
      white++;
  }
  for(int i = 0; i < n; i++){
    if(s[i] != '#')
      black++;
  }
  
  int min = n;
  if(white > black)
    min = black;
  else
    min = white;
  
  for(int i = 0; i < n-1; i++){
    int count = 0;
    for(int j = 0; j <= i; j++){
      if(s[j] != '.')
        count++;
    }
    for(int k = i+1; k < n; k++){
      if(s[k] != '#')
        count++;
    }
    if(min > count)
      min = count;
  }
  
  printf("%d", min);
  return 0;
}