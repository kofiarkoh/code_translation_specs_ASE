#include<stdio.h>
int main()
{
int q,i,x1,y1,x2,y2,x3,y3,x4,y4;
double m1,m2;
scanf("%d",&q);
for(i=1;i<=q;i++)
{
scanf("%d%d%d%d%d%d%d%d",&x1,&y1,&x2,&y2,&x3,&y3,&x4,&y4);
if((x2-x1)==0 && (x4-x3)==0)
{
printf("2\n");
}
else if(((x2-x1)==0 && (y4-y3)==0)||((x4-x3)==0 && (y2-y1)==0))
{
printf("1\n");
}
else if((x2-x1)==0 || (x4-x3)==0)
{
printf("0\n");
}
else
{
m1=(y2-y1)/((x2-x1)*1.00);
m2=(y4-y3)/((x4-x3)*1.00);
if(m1==m2)
{
printf("2\n");
}
else if(m1*m2==-1)
{
printf("1\n");
}
else
{
printf("0\n");
}
}
}
return 0;
}
