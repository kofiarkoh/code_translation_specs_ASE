//
//  main.c
//  Pablo Squarson's Headache
//
//  Created by x15071xx on 2016/06/03.
//  Copyright ?? 2016??´ AIT. All rights reserved.
//

#include <stdio.h>

int main(int argc, const char * argv[]) {
    
    
    int canvas[300][300]={0},position[200][2];
    int i,j,n=0,w=0,h=0,ni,di;
    while (1) {
        scanf("%d ",&n);
        if (n==0) {
            
            break;
        }
        canvas[149][149] = 1;
        //height width
        position[0][0]=149;position[0][1]=149;
        for (i=1; i<n; i++) {
            scanf(" %d %d ",&ni,&di);
            if (di==0) { //left
                canvas[position[ni][0]][position[ni][1]-1] = 1;
                position[i][0] = position[ni][0];
                position[i][1] = position[ni][1]-1;
            } else if (di==1) {//bottom
                canvas[position[ni][0]+1][position[ni][1]] = 1;
                position[i][0] = position[ni][0]+1;
                position[i][1] = position[ni][1];
            } else if (di==2) {//right
                canvas[position[ni][0]][position[ni][1]+1] = 1;
                position[i][0] = position[ni][0];
                position[i][1] = position[ni][1]+1;
            } else if (di==3) {//top
                canvas[position[ni][0]-1][position[ni][1]] = 1;
                position[i][0] = position[ni][0]-1;
                position[i][1] = position[ni][1];
            }
        }
        //width
        for (i=0; i<300; i++) {
            for (j=0; j<300; j++) {
                if (canvas[i][j] == 1) {
                    h++;
                    //printf("%d %d",i,j);
                    break;
                }
            }
        }
        //height
        for (i=0; i<300; i++) {
            for (j=0; j<300; j++) {
                if (canvas[j][i] == 1) {
                    w++;
                    break;
                }
            }
        }
        
        printf("%d %d\n",w,h);
        w=0;
        h=0;
        
        for (i=0; i<300; i++) {
            for (j=0; j<300; j++) {
                canvas[i][j]=0;
            }
        }
    }
    return 0;
}