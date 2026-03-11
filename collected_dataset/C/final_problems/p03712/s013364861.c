#include<stdio.h>
int main()
{
	char x[1000][100];
	int h,w,i,j;
while(scanf("%d %d",&h,&w)!=EOF)
{	for(i=0;i<h;i++)
{	getchar();
	for(j=0;j<w;j++)
	scanf("%c",&x[i][j]);}
for(i=0;i<w+2;i++)
printf("#");
printf("\n");
	for(i=0;i<h;i++)
printf("#%s#\n",x[i]);
for(i=0;i<w+2;i++)
printf("#");
printf("\n");}
return 0;
}