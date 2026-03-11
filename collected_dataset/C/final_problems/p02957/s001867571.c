#include<stdio.h>
int main()
{
    int A,B,sum=0;
    while(scanf("%d%d",&A,&B)!=EOF){
    sum=(A+B)/2;
    if(A+B==sum*2)
        printf("%d\n",sum);
    else
        printf("IMPOSSIBLE\n");
    }
    return 0;

}
