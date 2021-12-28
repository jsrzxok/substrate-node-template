## 常见错误汇总

### associated type `AccountId` not found

![image-20211227194537632](assets/image-20211227194537632.png)

加上这个：

![image-20211227194601021](assets/image-20211227194601021.png)

### the trait `TypeInfo` is not implemented for `Kitty`

![image-20211227194654356](assets/image-20211227194654356.png)

![image-20211227195600776](assets/image-20211227195600776.png)

### use of undeclared crate or module `sp_io`

![image-20211227213308592](assets/image-20211227213308592.png)

复制并添加：

![image-20211227213227178](assets/image-20211227213227178.png)

### associated type `Randomness` not found

![image-20211227214022567](assets/image-20211227214022567.png)



### cannot find trait `Randomness` in this scope

![image-20211228095631903](assets/image-20211228095631903.png)

![image-20211228095545517](assets/image-20211228095545517.png)

解法：

![image-20211228095720006](assets/image-20211228095720006.png)

### use of undeclared type `Currency`

### not found in `Config`

![image-20211228153316183](assets/image-20211228153316183.png)

![image-20211228153239146](assets/image-20211228153239146.png)



### missing `Currency` in implementation

![image-20211228153332409](assets/image-20211228153332409.png)

![image-20211228153531588](assets/image-20211228153531588.png)



