#include <stdio.h>
int map[4][3]={{1989,1,7},{1926,12,24},{1912,7,29},{1868,9,7}};
char map1[5][10]={"heisei","showa","taisho","meiji","pre-meiji"};
int sign;
void set(int search[]){
    int temp=sign;
    if(map[sign][0]>=search[0]){
        if(map[sign][0]>search[0])
            sign++;
        else if(map[sign][1]>search[1])
            sign++;
        else if(map[sign][1]==search[1] && map[sign][2]>=search[2])
            sign++;
    }
    if(sign==temp || sign==4)
        return;
    else
        set(search);;
}
int main(void){
    int line[3];
    for(;sign=0,scanf("%d %d %d",&line[0],&line[1],&line[2])!=EOF;){
        set(line);
        if(sign==4)
            printf("%s\n",map1[4]);
        else
            printf("%s %d %d %d\n",map1[sign],line[0]-map[sign][0]+1,line[1],line[2]);
    }
    return 0;
}