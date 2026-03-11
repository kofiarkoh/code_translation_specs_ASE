#include <stdio.h>

int main(void){

    int age;

    scanf("%d",&age);

    if (age == 1){
        printf("Hello World");
    }
    else{ 
        int A,B;
        scanf("%d%d",&A,&B);
        printf("%d",A + B);
    }
    return 0;

}