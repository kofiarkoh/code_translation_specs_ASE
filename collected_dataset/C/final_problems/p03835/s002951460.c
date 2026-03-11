#include <stdio.h>
int main(void){
    int k,s;
    scanf("%d%d",&k,&s);
    int count = 0;
    for(int i=0;i<k+1;i++)
    {
        for(int j=0;j<k+1;j++)
        {
            if(0<=s-i-j && s-i-j<=k)
            {
                count++;
            }
        }
    }
    printf("%d\n",count);
    return 0;
}