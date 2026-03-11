#include <stdio.h>
#include <stdlib.h>

typedef struct{
    int x;
    int y;
} treasure;
int main(void){
    int number_of_treasures;
    int number_of_regions;
    scanf("%d %d", &number_of_treasures, &number_of_regions);
    treasure* treasures = malloc(sizeof(treasure)*number_of_treasures);
    int i,j;
    for(i=0;i<number_of_treasures;i++){
        scanf("%d %d", &(treasures[i].x),&(treasures[i].y));
    }
    for(i=0;i<number_of_regions;i++){
        int x1,x2,y1,y2;
        scanf("%d %d %d %d", &x1, &y1, &x2, &y2);
        int count=0;
        for(j=0; j<number_of_treasures;j++){
            if(x1<=treasures[j].x&&treasures[j].x<=x2&&y1<=treasures[j].y&&treasures[j].y<=y2) count++;
        }
        printf("%d\n", count);
    }
    return 0;
}