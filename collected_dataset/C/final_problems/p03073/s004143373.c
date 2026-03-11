#include <stdio.h>
#include <string.h>
int main(void){
    // Your code here!
    char S[100001];
    int N, bstart=0, wstart=0; //black start, white start
    scanf("%s",S);
    N = strlen(S);
    for(int i = 0; i < N; i++)
    {
        if(i%2 == 1)
        {
            if(S[i] == '1')
                wstart++;
            else
                bstart++;
        }
        else if(i%2 == 0)
        {
            if(S[i] == '1')
                bstart++;
            else
                wstart++;
        }
    }
    printf("%d\n",(bstart<wstart)? bstart:wstart);
}
