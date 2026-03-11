/******************************************************************************

Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, PHP, Ruby, 
C#, VB, Perl, Swift, Prolog, Javascript, Pascal, HTML, CSS, JS
Code, Compile, Run and Debug online from anywhere in world.

*******************************************************************************/
#include <stdio.h>

int main()
{
    int N;
    int D1[100];
    int D2[100];
    int i;
    int count = 0;
    int flag = 0;
    
    scanf("%d", &N);
    
    for(i=0;i<N;i++){
        scanf("%d %d", &D1[i],&D2[i]);
    }


    for(i=0;i<N;i++){
        if(D1[i] == D2[i]){
            count++;
            if(count>=3){
                flag =1;   
            }
        }
        if(D1[i] != D2[i]){
            count = 0;
        } 
    }

 
    if(flag==1){
        printf("Yes");
    }
    else{
        printf("No");
    }

    return 0;
}
