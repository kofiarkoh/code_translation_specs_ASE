#include<stdio.h> 
#include <ctype.h>

int main(){
 
  int i;
  char str[1024] ;

  scanf("%[a-zA-Z .]", &str);

  for(i=0;str[i] != '\0';i++){
    str[i] = toupper(str[i]);
  }
  printf( "%s\n", str );

    return 0;
}