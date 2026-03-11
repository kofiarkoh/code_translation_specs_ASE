#include <stdio.h>
 
int main(void) 
{
    char c;
    int sum = 0;
    while ((c=getc(stdin))!= EOF) 
    {
        if('0' <= c && c <='9') 
        {
            sum += (c-'0');

            if(sum == 0)
            { 
                break;
            }
        } 
        else 
        {
            printf("%d\n", sum);
            sum = 0;
        }
    }
    return 0;
}
