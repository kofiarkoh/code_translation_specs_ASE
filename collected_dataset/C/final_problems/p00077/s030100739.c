#include <stdio.h>
 
int main(void) {
	int i,j;
    char str[100];
    while(scanf("%s",str) != EOF){
        for(i = 0;str[i] != 0;i++){
            if(str[i] == '@'){
                for(j = 0;j < str[i + 1] - '0';j++) printf("%c",str[i + 2]);
                i += 2;
            }
            else printf("%c",str[i]);
        }
        printf("\n");
    }
 
	return 0;
}
 