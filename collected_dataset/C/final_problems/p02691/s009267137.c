#include <stdio.h>
//#include <stdlib.h>
unsigned int height[200005];
int main()
{
   unsigned int n, i,j, number = 0,min=0,max=0;
   scanf("%u", &n);

    for(i = 0; i < n; i++)
    {
        scanf("%u", &height[i]);
        if(height[i]>max)max=height[i];
        if(height[i]<min||i==0)min=height[i];

    }
   // printf("%u\n",min);

    for(i = 0; i < n-1; i++)
    {
        j=(2*min>height[i])?i+2*min:i+height[i];

        for( ; j < n&&(j - i)<=2*max; j++)
        {
            if((j - i) == height[i] + height[j])
            {
                number++;
            }
        }
    }

    printf("%u\n", number);
    return 0;
}
