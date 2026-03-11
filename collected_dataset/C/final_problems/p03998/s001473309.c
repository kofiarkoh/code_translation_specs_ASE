#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

int main () {
    int a = 0;
    int b = 0;
    int c = 0;
    char ary_a[128];
    char ary_b[128];
    char ary_c[128];
    char turn = 'a';

    fgets(ary_a,sizeof(ary_a),stdin);
    fgets(ary_b,sizeof(ary_b),stdin);
    fgets(ary_c,sizeof(ary_c),stdin);
    //printf("%s%s%s", ary_a, ary_b, ary_c);

    //printf("%c\n", ary_a[0]);

    while(1){
        //printf("turn: %c\n", turn);
        //printf("p[%d]: %c\n", tmp, p[tmp]);

        if(turn == 'a'){
            turn = ary_a[a];
            if(isspace(ary_a[a]) != 0){
                puts("A");
                return 0;
            }
            a++;
        }else if(turn == 'b'){
            turn = ary_b[b];
            if(isspace(ary_b[b]) != 0){
                puts("B");
                return 0;
            }
            b++;
        }else{
            turn = ary_c[c];
            if(isspace(ary_c[c]) != 0){
                puts("C");
                return 0;
            }
            c++;
        }
    }

    return 0;
}
