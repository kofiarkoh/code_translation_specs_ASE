#include<stdio.h>
#include<ctype.h>
int main(void){
    int i,ascii = 0x61;
    char x;
    int counter[25] = {0};
    while(scanf("%c",&x) != EOF)
    {
        for(i = 0;i < 26;i++)
        {
            if(isupper(x))  x = tolower(x);
            if((int)x == i+ascii)    counter[i]++;
        }
    }
    for(i = 0;i < 26;i++)
    {
        printf("%c : %d\n",i+ascii,counter[i]);
    }
}
