#include <stdio.h>
#include <string.h>

#define NUMBER 200000

void swap_str(char s1[], char s2[])
{
    char tmp[NUMBER];
    int i;
    
    for (i = 0;(tmp[i] = s1[i]) != '\0';i++)
        ;
    for (i = 0;(s1[i] = s2[i]) != '\0';i++)
        ;
    for (i = 0;(s2[i] = tmp[i]) != '\0';i++)
        ;
}

int main(void)
{
    char s[NUMBER][10];
    int c[NUMBER] = {1};
    int n;
    int i,j;
    
    scanf("%d",&n);
    for (i = 0;i < n;i++){
        scanf("%s",s[i]);
    }
    
    for (i = 0;i < n-1;i++){
        for (j = 0;j < n-1;j++){
            if (strcmp(s[j],s[j+1]) > 0){
                swap_str(s[j],s[j+1]);
            }
        }
    }
    
    int M = 1;
    for (i = 0;i < n-1;i++){
        if (strcmp(s[i],s[i+1]) == 0){
            c[i+1] = c[i] + 1;
        }
        else{
            c[i+1] = 1;
            }
        if (c[i+1] > M){
            M = c[i+1];
        }
    }
    
    
    for (i = 0;i < n;i++){
        if (c[i] == M)
            printf("%s\n",s[i]);
    }
    return 0;
}
