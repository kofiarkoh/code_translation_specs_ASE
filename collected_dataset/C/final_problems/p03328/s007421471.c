/*  ex4_3
    Graku   */
#include<stdio.h>
int get_snow_depth(int len_above_west, int len_above_east);

int main(void) {
    int len_above_west, len_above_east;
    scanf("%d %d", &len_above_west, &len_above_east);

    printf("%d\n", get_snow_depth(len_above_west, len_above_east));

    return 0;
}

int get_snow_depth(int len_above_west, int len_above_east) {
    int tower_num = len_above_east - len_above_west; //higher(east) tower's number
    int tower_hight = (1 + tower_num)*tower_num / 2; // higher(east) tower's height

    return tower_hight - len_above_east; //depth of snow
}