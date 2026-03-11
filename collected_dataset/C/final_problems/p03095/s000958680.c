#include<stdio.h>
int main(){
 int n, i, a=1;
 scanf("%d", &n);
 int s[26];
 for(i=0; i<26; ++i) s[i]=1;
 for(i=0, getchar(); i<n; i++) s[getchar()-'a']++;
 for(i=0; i<26; i++) a=s[i]*a%(1000000000+7);
 printf("%d", a-1);
 return 0;
}