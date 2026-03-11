/*
  AOJ 0133
  Title:Rotation of a Pattern
  @kankichi573
*/
#include <stdio.h>
#include <string.h>
char pat1[8][9];
char pat2[8][9];
void rot()
{
        int i,j;

        for(i=0;i<8;i++)
                for(j=0;j<8;j++)
                        pat2[i][j]=pat1[7-j][i];
}
void display()
{
        int i;

        for(i=0;i<8;i++)
                printf("%s\n",&pat2[i][0]);
}
int main()
{
        int i;
        for(i=0;i<8;i++)
                scanf("%s",&pat1[i][0]);

        for(i=1;i<=3;i++)
        {
                rot();
                printf("%d\n",i*90);
                display();
                memcpy(pat1,pat2,8*9);
        }

  return(0);
}