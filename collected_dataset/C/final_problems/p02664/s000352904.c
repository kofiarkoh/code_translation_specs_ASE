#include<stdio.h>
#include<string.h>
int main(void){
    char T[20000];
    char Q[] = {'?','D'};
    int i,t,q,j;
    scanf("%s",T);
    t = strlen(T);
    for ( i = 0; i < t; i++)
    {
        if (T[i]==Q[0])
        {
            if(T[i+1]==Q[1]){
                T[i]='P';
            }
            else
            {
                T[i]='D';
            }
            
        }    
    }
    for ( j = 0; j < t; j++)
    {
        if (j>=1){
            if(T[j]==Q[1]&&T[j+1]==Q[1]&&T[j-1]==Q[1]){
                T[j]='P';
            }
        }    
    }
    
    printf("%s",T);
}