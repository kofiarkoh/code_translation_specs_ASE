#include <stdio.h>
int main(void){
    char w[101];
    int al[26]={0};
    int i;
    scanf("%s",w);
    for(i=0;w[i]!='\0';i++){
        al[w[i]-97]++;
    }
    for(i=0;i<26;i++){
        if(al[i]%2!=0){
            printf("No\n");
            break;
        }
    }
    if(i==26) printf("Yes\n");
}
