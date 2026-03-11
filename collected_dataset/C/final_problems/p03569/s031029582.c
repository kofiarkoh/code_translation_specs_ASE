#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int main(void){
    int count=0,ans=0;
    char s[300000],t[300000];
    scanf("%s",s);
    for (int i=0; i<strlen(s); i++){
        if (s[i]!='x'){
            t[count]=s[i];
            count++;
        }
    }
    t[count]='\0';
    for (int i=0; i<strlen(t)/2; i++){
        if (t[i]!=t[strlen(t)-1-i]){
            printf("-1\n");
            return 0;
        }
    }
    int n=strlen(s);
    for (int i=0; i<n/2; i++){
        if (s[i]!=s[n-1-i]){
            if (s[i]=='x'){
                ans++;
                n++;
            } else {
                ans++;
                n--;
                i--;
            }
        }
    }
    printf("%d\n",ans);
	return 0 ;
}