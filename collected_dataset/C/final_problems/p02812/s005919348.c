#include<stdio.h>
int main()
{
    int N,count=0;
    scanf("%d",&N);

    char S[100];
    scanf("%s",&S);

    for(int i=0; i<N; i++)
    {
        if(S[i] == 'A' && S[i+1] == 'B' && S[i+2] == 'C')
        {
            count++;
        }
    }

    printf("%d\n",count);


}