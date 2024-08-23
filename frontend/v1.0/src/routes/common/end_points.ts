enum End_points {
    SignIn = "/sign-in",
    SignUp = "/sign-up",
    Index = "/index",
    List = "list-all",
    New = "new",
    Detail = "detail",
    Post = "/post/*",
    User = "/user/*",
    Profile = "/user/profile",
    PostList = `/post/${End_points.List}`,
    PostNew = `/post/${New}`,
    PostDetail = `/post/${Detail}`
}

export default End_points;