/* ABC131-B
   Yuta Takayanagi  */

#include<stdio.h>
#include<math.h>
int main(void){
    int number, l, taste, pie, i;

    i = 0;
    pie = 0;

    scanf( "%d %d" , &number, &l);

    taste = l + i;

    for(i=1; i < number; i++){

        if(fabs(taste) > fabs(l+i)){

            pie += taste;
            taste = l + i;

        }else{

            pie += l + i;
            
        }

    }

    printf( "%d" , pie);

}