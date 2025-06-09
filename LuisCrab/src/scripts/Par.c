#include <stdio.h>
int par(int number){
    if(number % 2 == 0){
        return 1;
    }
    return 0;
}
int main(){
     int x = par(5);
     printf("%d\n",x);
     return 0;
}

