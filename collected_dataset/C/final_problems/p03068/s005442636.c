#include <stdio.h>

int main(int argc, const char * argv[]) {
    char s[10];
    int n=0,k=0,i=0;
    scanf("%d",&n);
    scanf("%s",s);
    scanf("%d",&k);
    for(i=0;i<n;i++){
        if(s[k-1] != s[i]){
            s[i] = '*';
        }
    }
    printf("%s",s);
    return 0;
}
