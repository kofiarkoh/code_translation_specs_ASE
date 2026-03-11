#include <stdio.h>
#include <string.h>

int main(void)
{
  char str[1000]={0};
  char res[1000][33]={0};
  char *p;
  int max, i, j, index, nmax;
  char (*mood)[33]={NULL};
  char (*len)[33]={NULL};
  max = nmax = index = 0;

  fgets(str, 1000, stdin);

  p = strtok(str, " ");

  while(p != NULL){
    strcpy(res[index], p);
    p = strtok(NULL, " \n");
    index++;
  }
  for(i = 0; i < index - 1; i++){
    for(j = 1; j < index; j++){
      if(strcmp(res[i], res[j]) == 0)
	nmax++;
    }
    if(nmax > max){
      max = nmax;
      mood = &res[i];
      nmax = 0;
    }
  }
    max = 0;
  for(i = 0; i < index; i++){
    if(max < strlen(res[i])){
      max = strlen(res[i]);
      len = &res[i];
    }
  }
  printf("%s %s\n", mood, len);
 
  return 0;
}