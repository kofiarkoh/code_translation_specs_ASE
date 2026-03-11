#include <stdio.h>

int main(){
    int A;
    int B;

    scanf("%d %d",&A,&B);

    int hasil = A+B;

    if(hasil>=24){
        hasil=hasil-24;
        printf("%d",hasil);
    }else{
        printf("%d",hasil);
        }



    return 0;
}
