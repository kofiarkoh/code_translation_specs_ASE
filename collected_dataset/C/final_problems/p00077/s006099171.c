#include <stdio.h>
#include <string.h>

int main()
{
  int j;
  char str[100];
  while (scanf("%s",str)!=EOF){
    int len=strlen(str);
    int i=0;
    while (i<len){
      if (str[i]=='@'){
        for (j=0;j<(str[i+1]-'0');j++)
          printf("%c",str[i+2]);
        i+=3;
      }
      else{
        printf("%c",str[i]);
        i++;
      }
    }
    printf("\n");
  }

  return 0;
}