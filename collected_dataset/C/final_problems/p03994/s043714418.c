#include <stdio.h>
#include <string.h>

int main(void){
    int i, j;
    unsigned int k;
    char s[100001];
    int len;

    scanf("%100000s",s);
    scanf("%d", &k);
	len = strlen(s);
    

    for(i=0; i<len; i++){
        if(k>= 123 - s[i]){    
            k -= 123 - s[i];
            s[i] = 'a';
        }
    }
    
     if( k > 0){
            s[len - 1] += k % 26; 
        }

   printf("%s", s);
   putchar('\n');
	return 0;
}