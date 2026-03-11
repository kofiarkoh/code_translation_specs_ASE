#include <stdio.h>


int number_of_str(const char c[]){

        int n = 0;
    
        for(int i=0;c[i] != '\0';i++){
                n++;
        }   
return n;
}


int main(void){

        int judge = 0;
        char c[1000];

        scanf("%s",c);

        int n = number_of_str(c);


for(int i=0;i<n-1;i++){
        if(c[i] == 'A' && c[i+1] == 'C'){
                printf("Yes\n");
                judge = 1;
                break;
        }   
}

if(judge != 1){ 
        printf("No\n");
}

return 0;
}

