#include <stdio.h>
#include <stdlib.h>
 
typedef struct futon{
    int x;
    int y;
    char dir;
    int alignment;//??????????????????
    int isChecked;
    int number_of_adjacent;
    struct futon* adjacent[6];
    int adjacent_type[6];
} futon;
futon futons[20000];
int isAdjacent(futon* a, futon* b);
int check(futon* object);
int comp(const void *a, const void *b);
 
int main(void){
    int number_of_futon;
    while(scanf("%d", &number_of_futon),number_of_futon){
        int i,j;
        int isNo=0;
        for(i=0;i<number_of_futon;i++){
            scanf("%d %d %c", &(futons[i].x), &(futons[i].y), &(futons[i].dir));
            futons[i].alignment=-1;
            futons[i].number_of_adjacent=0;
            futons[i].isChecked=0;
        }
        qsort(futons, number_of_futon, sizeof(futon), comp);

        for(i=0;i<number_of_futon;i++){
            int x=futons[i].x;
            for(j=0;j<i;j++){
                if(futons[j].x>=x-2) break;
            }
            for(;j<i;j++){
                if(futons[j].x>x+2) break;
                int adjacent_type=isAdjacent(&futons[i], &futons[j]);
                if(adjacent_type){
                    futons[i].adjacent[futons[i].number_of_adjacent]=&futons[j];
                    futons[i].adjacent_type[futons[i].number_of_adjacent]=adjacent_type;
                    futons[i].number_of_adjacent++;
                    futons[j].adjacent[futons[j].number_of_adjacent]=&futons[i];
                    futons[j].adjacent_type[futons[j].number_of_adjacent]=adjacent_type;
                    futons[j].number_of_adjacent++;
                }
            }
        }
        //?????°??????????????????
        for(i=0;i<number_of_futon;i++){
            if(futons[i].isChecked) continue;
            if(check(&futons[i])==-1){
                break;
            }
        }
        if(i==number_of_futon){
            printf("Yes\n");
        }else{
            printf("No\n");
        }
    }
    return 0;
}

int isAdjacent(futon* a, futon* b){//??\?§?????????????????????°0
    int Ax1, Ax2, Ay1, Ay2, Bx1, Bx2, By1, By2;
    Ax1=a->x;
    Ay1=a->y;
    if(a->dir=='x'){
        Ax2=a->x+1;
        Ay2=a->y;
    }else{
        Ax2=a->x;
        Ay2=a->y+1;
    }
    Bx1=b->x;
    By1=b->y;
    if(b->dir=='x'){
        Bx2=b->x+1;
        By2=b->y;
    }else{
        Bx2=b->x;
        By2=b->y+1;
    }
    if(abs(Ax1-Bx1)+abs(Ay1-By1)==1) return 1;
    if(abs(Ax1-Bx2)+abs(Ay1-By2)==1) return 2;
    if(abs(Ax2-Bx1)+abs(Ay2-By1)==1) return 2;
    if(abs(Ax2-Bx2)+abs(Ay2-By2)==1) return 1;
    return 0;
}
int check(futon* object){
    if(object->isChecked){
        return 0;
    }
    object->isChecked=1;
    if(object->number_of_adjacent==0){
        return 0;
    }
    if(object->alignment==-1){
        object->alignment=1;
    }
    int i;
    for(i=0;i<object->number_of_adjacent;i++){
        switch(object->adjacent_type[i]){
            case 1:
                if(object->adjacent[i]->alignment==-1){
                    object->adjacent[i]->alignment=object->alignment;
                }else{
                    if(object->adjacent[i]->alignment!=object->alignment) return -1;
                }
                break;
            case 2:
                if(object->adjacent[i]->alignment==-1){
                    object->adjacent[i]->alignment=object->alignment^1;
                }else{
                    if(object->adjacent[i]->alignment==object->alignment) return -1;
                }
                break;
        }
        if(object->adjacent[i]->isChecked==0){
            if(check(object->adjacent[i])==-1) return -1;            
        }
    }
    return 0;
}

int comp(const void *c1, const void *c2){
    futon *a = (futon *)c1;
    futon *b = (futon *)c2;
    return a->x - b->x;
}