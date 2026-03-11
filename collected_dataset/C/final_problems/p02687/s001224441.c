#include <stdio.h>
#include <string.h>

int main(void)
{
    char buff[4];

    gets(buff);

    if(!strcmp(buff, "ABC")){
      printf("ARC\n");
    }else
        printf("ABC\n");
    return(0);
}