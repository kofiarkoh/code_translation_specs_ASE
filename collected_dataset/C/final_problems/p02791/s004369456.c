#include<stdio.h>
int main()
{
    int x,y,count=1,min;
    scanf("%d",&y);
    scanf("%d",&x);
    y--;
    min=x;
    while(y--)
        {
        scanf("%d",&x);
    if(min>=x)
    {
        count++;
        min=x;
    }

    }
printf("%d",count);

return 0;

}
